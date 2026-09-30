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
  `host.run`. A missing run, a launch-failure or not-executed run, and an
  unreadable store each refuse with the declaration remedy. An existing run of
  another task identity is admitted, with that identity shown.
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
