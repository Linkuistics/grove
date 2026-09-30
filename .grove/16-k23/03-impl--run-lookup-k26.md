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
- This node's `Done when` holds. As this leaf's last act, cut the node's
  `review-impl`: run `grove-llm leaf-add dispatch-records-k23 dispatch-records
  --kind review-impl`, with `**Reviews:** dispatch-records-k23`. Write the node
  brief's review doubts into its body.
