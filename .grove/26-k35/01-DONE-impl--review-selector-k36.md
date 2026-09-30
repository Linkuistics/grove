# review-selector-k36

## Goal

Ship the example review policy's reusable selector. Given a generic reviewed
artifact with a `run` or `declared` creator, it selects a configured review
candidate from a different provider origin, or refuses with the exact remedy.
No task file or Grove convention is involved.

## Context

The spec's `#review-policy` owns the checks and refusals. The caller-context
`reviewedArtifact` shape arrived with `bounded-context-k22`, and run lookup
with `run-lookup-k26`. The selector is TypeScript in the shipped examples, not
Rust.

## Done when

- The example policy lists exact review-kind entries that apply the rule, and
  routes other kinds through the owner's static table. A custom review label
  applies the rule only when the owner lists it.
- Run reference: the selector resolves the run's recorded provider through
  `host.run`. A missing run and a launch-failure or not-executed run each
  refuse with the declaration remedy. An unreadable store never reaches the
  selector: dispatch refuses it first, exit 4, with the store's own remedy
  (`run-lookup-k26`). An existing run of another task identity is admitted,
  with that identity shown.
- Declared reference: the declared provider is used and is labelled declared in
  inspection and the run record.
- Membership is exact and case-sensitive against the current catalog's origins,
  with no normalisation. A non-member refuses with a correction remedy. The
  chosen candidate's origin must differ. A same-origin candidate behind a
  gateway label refuses. The selector never selects a replacement.
- Command-seam tests with the actual shipped example cover: a different-origin
  reviewer on repeated invocations, retries and explicit choice; same-origin and
  gateway-disguise refusal; an unknown run; a run marked not executed; declaration
  adoption; a relabelled origin and a misspelt declaration refusing as
  non-members; and the generic form selecting with no task file.
- A changed current mapping does not change the provider resolved from an
  existing run.

## Decisions (running log)

**Specifier and file.** The selector ships as `harness-dispatch/examples/review`,
from `worker/examples/review.ts`. The digest set, the declarations build and
the readable-source copy already take every `worker/examples/*.ts`, and
`documented_specifiers` discovers it. What does not follow by itself is the
`embedded` table in `worker/src/main.ts` and the archive manifest in
`scripts/release-common.sh`, which `scripts/release.test.sh` diffs against
`dispatch.sh build`; both change with it.

**A review entry maps the creator's origin to a reviewer.** Each configured
review kind has an entry `{ <creator origin>: <candidate ID> }`, rather than
one reviewer per kind. The owner's own Grove configuration switches which
provider leads between arrangement profiles (`codex-led`, `claude-led`, and the
two mixed ones), so a producer is routinely finished under one arrangement and
reviewed under another. A single reviewer per kind would refuse every such
review; keyed by the recorded origin, the table picks the other provider. It
is still an exact table with no fallback: an origin the entry lacks refuses as
an incomplete mapping, and a mapped or chosen candidate of the creator's own
origin refuses rather than being replaced by another candidate. That is the
root brief's "the supplied configuration chooses another provider". An explicit
choice replaces the entry's pick, and the same checks apply to it.

**Order of the checks.** Reviewed artifact present, creator present, creator
resolved (a run looked up, found and without a launch failure, or a
declaration), the resolved origin an exact member of the current catalog's
origins, then the candidate (the choice, else the entry's pick for that
origin), then the candidate's origin differs. Membership precedes the pick, so
a relabelled or misspelt origin refuses as a non-member and is never compared
as "different".

**Lookup does not depend on the kind.** `lookUpCreator(context, host)` looks up
whatever creator run the context names, so the Grove adapter (k37) can compose
it after its own loader. The example's loader returns the caller's context, or
an empty version-1 context when there is none, since a loader that returns
nothing refuses.

**A reviewed artifact under an unlisted kind refuses.** The generic analogue of
the adapter's `**Reviews:**`-under-an-unlisted-kind rule, and the spec's
"custom review labels require an explicit example-policy entry": the presence
of `reviewedArtifact` is the caller saying this is a review. Without one, an
unlisted kind takes the owner's static table, where the rule does not apply.

**The example builds on the static example.** Its catalog is the static
example's four candidates (`your-provider`) plus a second origin's harness for
reviews and one gateway candidate that reaches `your-provider` through
`my-gateway-wrapper`; its routes are the static example's. The gateway
candidate is the shipped gateway-disguise case.

**A real not-executed run, not a store fixture.** The review tests need a run
cancelled at the linearization point. Rather than insert a launch-failure row
by hand, the handoff tests' stall moved into `tests/support/stall.rs`,
generalised over the run's arguments; `handoff.rs` keeps a wrapper, so its
call sites and its eight tests are unchanged. The exec-error run is real too,
from a wrapper naming a missing interpreter.

**The SDK's `LaunchFailure.cause` doc was wrong.** It named `not_executed` for
a cancelled launch; `src/run.rs` writes `cancelled`, as the spec and README
say. The shipped declaration now says so, since the selector's refusal quotes
the cause.

**The tests were seen to discriminate.** Nine wrong selectors, each applied to
the shipped source, rebuilt and run: membership dropped, case-folded,
the creator's origin read from today's catalog, a same-origin reviewer
replaced, a never-executed run admitted, a missing run admitted, an explicit
choice unchecked, an unlisted kind with a reviewed artifact routed, and routes
matched by inherited keys. Each turned exactly the tests aimed at it red, and
the source's digest matched after the sweep. Stripping the fixture's three
`@ts-expect-error` lines showed each guards its intended error.

**A flake outside this leaf became its own leaf.** A full package run failed
once in `tests/run.rs`'s descriptor-7 test, which this leaf does not touch;
it passed on every rerun. `descriptor-seven-flake-k56` carries it.
