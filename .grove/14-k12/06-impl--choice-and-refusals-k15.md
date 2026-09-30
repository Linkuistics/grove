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
  malformed CLI, 3 refusal, 4 required-record failure, 5 worker or protocol
  failure, 124 timeout, 126 and 127.
- Help carries independent-use and refusal-recovery examples. It has no pager,
  no interactive confirmation and no retry.
- The static starter examples give exact kind mappings. They explain effort by
  abstraction, uncertainty, consequences, downstream repair, reversibility and
  available checks, not as model rankings. Each is registered as a
  `harness-dispatch/examples/…` specifier with type declarations and a readable
  source. A test imports each from a temporary personal policy and selects
  through it.
- The package's usage documentation covers standalone inspect and run, policy
  authority, the static form, explicit choice, the required run record and
  `record show`. It states that `inspect` is a
  proposal: it is not a launch reservation, and evaluating trusted TypeScript
  is not promised to be free of side effects. The spec's notice states what is
  delivered.
- This node's `Done when` holds. Retiring this leaf closes the node. As this
  leaf's last act, cut the node's `review-impl` as the node's sibling,
  directly after it, never inside it. Run `grove-llm leaf-insert --kind
  review-impl dispatch-delivery-k16 static-dispatch`, targeting the first root
  entry after this node (today `dispatch-delivery-k16`). Give it
  `**Reviews:** static-dispatch-k12`, and write into its body the doubts the
  node brief names.
