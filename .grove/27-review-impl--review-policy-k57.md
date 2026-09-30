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
