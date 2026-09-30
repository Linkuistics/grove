# review-policy-k57

**Reviews:** review-policy-k35

## Goal

An adversarial, inspection-only read of the whole `review-policy-k35` node:
the shipped review selector, `harness-dispatch/examples/review`
(`review-selector-k36`), and the Grove adapter, `harness-dispatch/grove`, with
the Grove review example that composes them,
`harness-dispatch/examples/grove-review` (`grove-review-adapter-k37`). Produce
findings, not fixes.

## Context

The provider rule is the first release's flagship requirement. A permissive
comparison, or a refusal that quietly admits, reads exactly like a working
policy in every test that does not target it. Read against the spec's
`#review-policy` and `#identity-and-creator`, the shipped-examples row of
`#test-seams`, and `docs/adr/a-review-carries-its-creator-reference.md`. Each
leaf's running log records why it chose as it did, and the mutation sweeps it
ran: each wrong selector or adapter turned exactly the tests aimed at it red.
A mutation shows only that a test notices the break it was written for. The
review's job is the break nobody wrote a test for. `creator-reference-k38`
builds the methodology on this next, and its lifecycle cases run through it.

## What to doubt

- **The loader refusal is a core change made for the adapter.** A `loadContext`
  may now return `select`'s refusal shape, judged by the front whenever a
  loader's result has a `status` (`policy::loader_refused`,
  `choice::deliver`). Does anything else a loader can legitimately return
  carry `status`? Does the breach-first order hold on every path? Is its
  `input`, `source` and `location` the right evidence for an owner?
- **The adapter version is reported on import, not on use.** The worker names
  the embedded modules that bring the adapter by module identity
  (`bringsAdapter` in `worker/src/main.ts`). A future example that composes the
  adapter must join that set. The test that enumerates the examples' sources
  for `from "harness-dispatch/grove"` is meant to catch the omission: does it?
  Can a policy import the adapter without the worker seeing it?
- **The line grammar is strict and markdown-blind.** A marker line counts
  wherever it begins a line, a fenced example included, and a declared origin
  is taken verbatim. Is that the right side of each trade-off for real Grove
  review bodies, which quote conventions? Does any real review leaf in the
  owner's groves refuse under it?
- **The task file is read for every kind, up to the context budget.** A leaf
  larger than the budget now refuses every dispatched session under the Grove
  example, not only reviews. Is the remedy (`--context-bytes` in the command)
  discoverable where that refusal lands?
- **The fake producer writes its own line.** The command-seam tests prove the
  adapter reads a line a producer wrote from `HARNESS_DISPATCH_RUN_ID`. They do
  not prove a real session writes it; that methodology is
  `creator-reference-k38`'s. Check the documentation does not promise it early.
- **Refusals that quietly admit.** For each check in the selector and the
  adapter, look for an input that should refuse and selects: near-miss kinds,
  inherited object keys, a run whose lookup failed but whose provider is still
  read, a declaration that matches after some normalisation.

## Done when

- Every doubt above has been read against the code and the tests, and each
  finding names its file, its line and a failure scenario, or the doubt is
  recorded as examined and found sound.
- A review with findings worth acting on cuts its `integrate-review-impl` leaf
  where `pick` reaches it next.

## Decisions (running log)

**Evidence boundary.** Review the two producer changes `cb5b12d9` and
`d3e44d41`, against the current source at `d3e44d41`. This is an
inspection-only review: no test, build, lint or format command runs, and no
production or test code changes. The graph project is the current workspace's,
but its generation is `2026-09-29T11:18:53Z`, before this package existed;
the exact reviewed package paths are `not_tracked`. Graph searches therefore
establish nothing about these additions. Direct source reads supply the
evidence, and excluded documentation is read directly too.

**Context-budget remedy.** The adapter passes the entire context budget as
the task read's `maxBytes`. An oversized file therefore reaches the generic
read-limit remedy with no room to increase `maxBytes`. That remedy tells the
owner to increase it up to the budget it already equals, and does not name
`--context-bytes`. This is actionable diagnostic work, not a request to relax
the bound; record it as F1.

**Creator maintenance is still pending.** The README and the shipped Grove
example describe sessions writing and removing the creator line as existing
behavior. The current plugin's `TASK-FORMAT.md` still excludes records of past
sessions, and `references/retire.md` says a waiting review needs no such record.
The adapter leaf explicitly defers that amendment to `creator-reference-k38`.
Record the premature usage promise as F2, without treating the scheduled
methodology work itself as a missing implementation in this node.

**Adapter-report regression guard.** The examples test identifies a composing
example by one direct-import substring. A new embedded example composing
`groveReviewSelector` via `harness-dispatch/examples/grove-review` has no such
substring. Omitting it from `bringsAdapter` then reports `null`, and the test
expects that same `null`. Record this unguarded documented composition path
as F3; it concerns the regression guarantee, not the current Grove example's
correct report.

## Findings

Coordinates below are against `d3e44d41`. Failure scenarios are inferred from
source, not dynamically reproduced in this review. Integration independently
triages them and owns every accepted fix and its executable verification.

### F1 — P2: Name the context-budget remedy when the task read already uses it all

**Location:** `crates/harness-dispatch/worker/grove/index.ts:125`, reaching
`crates/harness-dispatch/src/context.rs:453` through
`crates/harness-dispatch/src/choice.rs:424`.

The adapter reads with `maxBytes = request.limits.contextBytes`. On overflow,
`context_breach` takes the `Some(max)` branch because `max` equals the budget,
not the branch for a requested bound above it. `source_too_large` then says to
read with a larger `maxBytes`, up to that same context budget. There is no
larger admitted value, and the owner activated an embedded example rather
than a loader whose read they can edit.

**Failure scenario:** activate `grove-review` and supply a 300 KiB task file
under the default 256 KiB context budget, for `impl` or `review-impl`. The
session refuses, and its own remedy recommends an impossible increase instead
of naming `--context-bytes` in Grove's configured command. The README at
lines 766–768 gives the correct remedy, but the refusal does not point to it.
The new adapter creates this ordinary use of the older generic diagnostic.

The command-seam test
`a_task_file_that_cannot_be_read_refuses_naming_it` at `tests/grove.rs:793`
checks an overflow's code only. It does not establish that the owner can act
on the remedy when `maxBytes` already equals the budget. Preserve the whole-file
read and refusal; make the effective-budget remedy discoverable in the refusal.

### F2 — P2: Keep creator-line maintenance labelled pending until the methodology ships

**Location:** `crates/harness-dispatch/README.md:773`, also lines 705–711 and
`crates/harness-dispatch/worker/examples/grove-review.ts:36`.

The usage text states that a dispatched finishing session writes its run line
and a direct finishing session removes the line. That is the intended
methodology, but it is not this increment's delivered behavior.
`plugins/grove/skills/grove/TASK-FORMAT.md:141` still excludes records of past
sessions, and `plugins/grove/skills/grove/references/retire.md:13` explicitly
says a waiting review needs no record of its producer's session. The live
`creator-reference-k38` node owns amending those instructions.

**Failure scenario:** an owner follows the new activation instructions and
lets a real dispatched producer finish under the current skills. Its review
has no creator line and refuses, although the guide described the producer
as maintaining it. A pre-existing stale line is the more consequential case:
the current retirement instructions do not replace or remove it, so the
adapter can consume a reference to an earlier invocation. The ADR explicitly
accepts association as an attestation; it does not make stale references safe.

`tests/grove.rs:44` implements the line-writing convention in a fake wrapper,
and the producer/review test checks that wrapper. It proves the adapter and
record lookup, not behavior of real methodology-driven sessions. Do not move
k38's implementation into this review node; qualify the current usage promise
and make the interim responsibility explicit. The spec's initial delivered/
pending notice already distinguishes future design, whereas this README
passage does not.

### F3 — P3: Cover indirect adapter composition in the import-report regression guard

**Location:** `crates/harness-dispatch/tests/grove.rs:1026`, with the explicit
module set at `crates/harness-dispatch/worker/src/main.ts:99`.

The test infers whether an example brings the adapter from the literal
substring `from "harness-dispatch/grove"`. Its expected adapter report is
`null` for any example without that substring. The worker, meanwhile, requires
every embedded example that brings the adapter to join `bringsAdapter`, because
its bundled dependencies do not call the runtime module-registration callback.

**Failure scenario:** add an embedded example that imports and uses
`groveReviewSelector` from `harness-dispatch/examples/grove-review`, the public
composition path documented at `README.md:790`, and omit that new module from
`bringsAdapter`. Its adapter dependency was bundled; importing the new example
therefore reports `null`. The test sees no direct adapter-import substring
and expects that same `null`, so it blesses the missing report. The existing
direct-import mutation control does not reach this case. Single-quoted or
dynamic direct imports are further forms the substring does not classify.

The current shipped Grove example is explicitly registered and reports
correctly; this is a regression-coverage gap in the future-example guarantee
the task asks this review to assess. Use evidence that reaches the adapter
through the supported composition paths, without deriving expected absence
from a direct-import spelling alone.

## Disposition of the review doubts

1. **Loader refusal — sound in the inspected paths.**
   `src/context.rs:39` admits only the seven version-1 top-level context
   fields, so no legitimate top-level `status` is captured by the new branch
   in `choice::deliver`. A nested `facts.status` remains ordinary data.
   `policy::loader_refused` requires exactly the refusal fields, nonblank
   strings and `status: "refused"`; selected and mixed shapes refuse.
   `worker/src/main.ts:317` reports a recorded breach before returning the
   loader's value, both after a caught error and after an ordinary return.
   The front ends the context stage before calling `select`. The refusal
   names the policy source and `policy.loadContext`, and the input consistently
   names the explicit choice or kind. The adapter's message supplies the task
   file and relevant line. `a_loader_can_refuse_as_the_policy_and_nothing_is_selected`
   covers routes, callbacks, sync/async loaders, malformed shapes, caller
   context and a swallowed bound breach.

2. **Adapter reported on import — current behavior sound; guard gap F3.**
   The registration callback marks direct imports and the current Grove
   example. Policy, context and selection frames carry the report, and
   `Loaded::reported` keeps the latest valid report. The import-report test
   covers a side-effect import, import during `loadContext`, import during
   `select`, and the corresponding run record. A personal external helper
   importing the embedded adapter still uses that callback. The omission
   risk is another embedded example, whose dependencies are already bundled.

3. **Strict, markdown-blind grammar — a visible compatibility trade-off.**
   The code implements the spec's beginning-of-line markers, exactly one
   marker of each kind, canonical handle/run forms, and LF/CRLF handling.
   A declaration is returned verbatim and then checked for exact catalog
   membership. Fixtures cover fenced duplicates, prose/indented mentions,
   malformed lines and spaced/case-changed declarations. The trade-off has
   real examples: in the sibling `APIAnyware.add-ocaml-target` workspace,
   `.grove/183-k455/05-k581/22-k595/10-k612/02-DONE-review-impl--chez-library-absence-byte-check-k626.md`
   has its canonical relationship at line 3 and repeats a backticked
   relationship at line 48 in the review report. This adapter would refuse
   that body as duplicate even after adding a valid creator line. That
   specimen was read directly; its graph generation `2026-09-08T08:04:02Z`
   does not track the file. This is evidence of migration friction, not a
   request to introduce markdown parsing or silently prefer the first marker:
   the spec and duplicate refusal explicitly tell the owner to reword or
   indent quoted markers. No exhaustive compatibility claim is made.

4. **Whole task read for every kind — correct bound, remedy gap F1.**
   The read is necessary to detect an unlisted kind's relationship. The
   adapter reads only the supplied path, with the whole context budget, and
   the measured task digest enters inspection and the run record. The text
   itself does not enter delivered context. Missing, unreadable, non-UTF-8,
   non-file and oversized inputs refuse; no tree/brief traversal or identity
   recovery from filenames occurs. The tests' misleading filename, unprovided
   task ID and unreadable decoy files target those boundaries. F1 concerns
   the owner-facing remedy, not the chosen bound.

5. **Fake producer — correct seam, early documentation promise F2.**
   The wrapper writes the run dispatch actually exports, and the test later
   changes today's provider mapping and still reads the immutable creator
   snapshot. It also checks task digest, evidence class, adapter version and
   run-record output. The earlier-same-task-run case correctly refuses a
   missing line. These tests deliberately contain no Grove binary or
   methodology session; real lifecycle maintenance remains k38's.

6. **Refusals that admit — no bypass found in the inspected selector/adapter.**
   Exact own-key checks guard review kinds, ordinary routes and origin entries;
   a reviewed artifact under an unlisted kind refuses before taking a static
   route or accepting an explicit choice. The creator is required and looked
   up before provider membership and reviewer comparison. Missing runs and
   launch failures refuse, and a store-read error propagates out of
   `Loaded::context` without answering the worker or reaching selection.
   Provider membership is exact and case-sensitive, and same-origin/gateway
   candidates refuse without replacement, including explicit choices and
   retries. Different task identities and unknown execution remain admitted
   intentionally on the run-reference attestation. The command-seam tests
   cover those contracts; the producer logs record targeted mutation controls,
   which this review reads as prior evidence rather than fresh passing runs.

The adapter/selector remain explicit policy modules. Build digests include
their source; the declaration build, type fixtures, embedded table and archive
manifest include the new exports and files. These source checks do not assert
that package checks or per-target smoke were rerun or passed in this session.

## Handoff

`review-policy-k58` is the adjacent `integrate-review-impl` leaf for this
report, inserted before the live `creator-reference-k38` sibling node. It
reads this review by handle and independently triages the findings; no finding
is promoted to a settled implementation decision by this review.
