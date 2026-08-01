# doubt-grove-design-k3

**Kind:** design

## Goal

Produce the current-state design for composing doubt-driven development with
Grove review mechanics, including atomic mid-session promotion and advisory
review-target diversity.

## Context

Read the root brief and its pointers. Inspect existing tree mutation and launch
routing through the codebase graph before choosing mechanics. Create or rework a
durable spec and ADRs only where they earn their place; do not implement code.

## Done when

- The design maps behavior for every producer family, `review-*`,
  `integrate-review-*`, research pairs, and non-Grove sessions.
- One atomic operation is specified for promoting the picked plain producer
  while preserving its stable handle, structural grouping, order, and
  all-or-nothing behavior.
- The producer session's stop/commit/retire/signal sequence after promotion is
  unambiguous.
- The advisory warning defines which producer target it compares, when it is
  emitted, and how stateless restart semantics survive; it warns if either
  harness or model matches and never blocks launch.
- Canonical documentation/skill ownership and compatibility behavior are named.
- Test seams cover the CLI operation, routing decision, generated content, and
  unchanged standalone skill behavior.

## Notes

Use `linkuistics:codebase-design` for seam placement and
`linkuistics:decision-records` if a decision clears its when-to-write test.
