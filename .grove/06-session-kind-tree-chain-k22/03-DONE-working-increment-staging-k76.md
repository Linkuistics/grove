# working-increment-staging-k76

**Kind:** planning

## Goal

Make aggressive decomposition into multiple groves the default whenever the
work has obvious independently deliverable stages, while keeping changes that
must land together in one working increment.

## Context

- Apply the rule to both the Grove skill and this grove's remaining live task
  tree.
- Independence in code location is insufficient. A split is valid only when
  the earlier grove leaves the product working and delivers a useful,
  verifiable increment on which later groves can build.
- Conversely, do not keep a large impl/review/integrate chain merely because a
  single final design mentions all of it. Prefer dependency-ordered groves for
  schema, migration, lifecycle, cleanup, methodology, and documentation when
  each boundary can be made operationally sound.
- Preserve stable handles and explicit dependencies while restructuring the
  current grove; use `leaf-decompose`, `leaf-add`, and `leaf-insert` rather than
  hand-inventing tree shapes.

## Done when

- The Grove skill tells planning sessions to search actively for the smallest
  sequence of working increments and to create separate groves for obvious
  stages.
- The skill states the hard boundary: changes that cannot independently leave
  the system working stay together even if their code edits are separable.
- Every still-live concern in this grove is audited against that rule and any
  obvious multi-increment item is decomposed or replaced with dependency-ordered
  leaves/groves.
- The resulting tree records why retained combined work is atomic at the
  product/behavior level, not merely convenient to implement together.

## Notes

This is a throughput and context-size rule. Smaller working increments shorten
implementation, review, and integration sessions without sacrificing a green
handoff between them.

## Working-increment audit

- Decomposed `legacy-launch-removal-k46`, `legacy-review-removal-k47`,
  `methodology-and-viewer-k48`, `durable-docs-reconciliation-k49`, and
  `acceptance-verification-k50` into dependency-ordered children. Every child
  leaves a working product or independently useful verification/documentation
  handoff for its successor.
- Retained `tree-access-lock-k52`, `session-kind-migration-k27`,
  `driver-lease-k31`, `session-signal-path-k57`, `session-epoch-k35`,
  `lifecycle-cutover-k39`, and `finish-lifecycle-k43` as single producers.
  Their contracts are atomic at the behavior level: a partial lock, migration
  transaction, ownership epoch, authoritative cutover, or guarded finish would
  expose an incomplete protocol rather than a useful working increment.
- Retained `herdr-session-kind-viewer-k51` and `msrv-claim-k74` as standalone
  leaves because each already is one independently demonstrable increment.
- Retained each producer's linked review and integration beside its producer.
  An inspection-only review produces findings rather than product behavior, and
  its integration is licensed only to repair that same reviewed artifact; the
  three steps therefore do not form separate working increments.
