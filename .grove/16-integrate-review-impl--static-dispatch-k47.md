# static-dispatch-k47

**Integrates:** static-dispatch-k46

## Goal

Triage the findings in `static-dispatch-k46` against the committed standalone
`harness-dispatch` implementation and its contract. Apply the valid findings,
record the disposition of every finding, and verify the resulting behavior.

## Context

The producer is the completed `static-dispatch-k12` node. The review is anchored
to producer tip `e0d5c67d5e69`; read its own retired leaf for findings and source
evidence rather than treating this charter as a prescribed fix list.

The implementation lives in `crates/harness-dispatch/`. Its area contract is
`docs/specs/harness-selection-and-execution.md`, with the policy-ownership and
pre-exec evaluation ADRs cited by that spec. Read the producer's recorded
verification evidence and the package's current tests when judging the review.

## Done when

Every review finding has a supported disposition. Valid defects are corrected
with meaningful verification appropriate to their behavior and the required
project checks, and the integration is retired and committed. Any durable
decision made during triage is reconciled with the documentation that owns it.

## Notes

This task is placed immediately after the review so its cited implementation
does not change before triage. Later leaves already own the remaining delivery,
computed-context and evaluation-boundary increments; keep their contracts in
view when deciding what belongs in this integration.
