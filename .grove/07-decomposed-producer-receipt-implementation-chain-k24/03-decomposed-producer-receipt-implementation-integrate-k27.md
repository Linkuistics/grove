# decomposed-producer-receipt-implementation-integrate-k27

**Kind:** integrate-review-impl

**Integrates:** decomposed-producer-receipt-implementation-review-k26

## Goal

Verify and apply the implementation review's findings, leaving decomposed
producer handoff complete and internally consistent.

## Context

Read the design artifacts, the implementation diff, and findings in
`decomposed-producer-receipt-implementation-review-k26`. Reproduce each claim
before changing code or documentation.

## Done when

- Every finding is classified and each verified issue is fixed or its trade-off
  made visible.
- Focused and full relevant tests, formatting, and lints pass.
- Receipt parsing, lifecycle semantics, routing warnings, glossary,
  methodology, architecture, and user docs agree.
- No work required by the reviewed decomposed-producer contract remains unnamed
  in the tree.

## Notes

Externalise substantial redesign as a new producer review chain inside this
chain node rather than expanding this integration leaf.
