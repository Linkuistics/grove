# choice-and-refusals-k15

## Goal

Complete the static skeleton. An explicit `--choice` names one configured
candidate. Every refusal is actionable: a stable code, the stage, the input and a
remedy, plus the equivalent `inspect` invocation. The supplied static starter
examples ship as embedded package specifiers an owner can import.

## Context

The contract is in the spec's `#policy-and-choice` (explicit choice, static
examples) and `#diagnostics` sections. How `select` accepts or refuses a choice,
and `explicit_choice_mismatch`, are `computed-policy-k20`'s; refuse `select` as
before.

## Done when

- With `--choice ID`, a `routes` policy accepts any configured candidate,
  including one for a kind its table does not route. Inspection says the
  explicit choice selected it, not a route. An unknown ID refuses without
  selecting anything else.
- Every refusal before handoff carries a stable code, a stage, a message, the
  relevant input or source, and a remedy. With `--json`, a failure prints one
  JSON error on stderr and no partial stdout object. Text mode prefixes
  captured policy diagnostics on stderr.
- A refused `run` also reports the equivalent `inspect` invocation, without the
  prompt: a command line in text mode and an argv array in JSON.
- The exit results match the spec's table for the stages that exist: 2
  malformed CLI, 3 refusal, 5 worker or protocol failure, 126 and 127.
- Help carries independent-use and refusal-recovery examples. It has no pager,
  no interactive confirmation and no retry.
- The static starter examples give exact kind mappings. They explain effort by
  abstraction, uncertainty, consequences, downstream repair, reversibility and
  available checks, not as model rankings. Each is registered as a
  `harness-dispatch/examples/…` specifier with type declarations and a readable
  source. A test imports each from a temporary personal policy and selects
  through it.
- The package's usage documentation covers standalone inspect and run, policy
  authority, the static form and explicit choice. It states that `inspect` is a
  proposal: it is not a launch reservation, and evaluating trusted TypeScript
  is not promised to be free of side effects. The spec's notice states what is
  delivered.
- This node's `Done when` holds. As this leaf's last act, cut the node's
  `review-impl`: run `grove-llm leaf-add static-dispatch-k12 static-dispatch
  --kind review-impl`, with `**Reviews:** static-dispatch-k12`. Write into its
  body the doubts the node brief names.
