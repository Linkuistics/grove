# kind-routing-k6

**Kind:** work

## Goal
Execute plan Task 5: GROVE_<KIND>_HARNESS reroutes leaves of a kind to another
harness; model resolution follows the post-override harness; unknown names
fail loudly; GROVE_HARNESS_BIN_<NAME> test seam.

## Context
- Plan Task 5: docs/superpowers/plans/2026-07-18-codex-pi-harness-switch.md
- Consumes helpers from scoped-model-envs-k5.

## Done when
review_leaf_reroutes_to_the_review_harness and
unknown_review_harness_fails_loudly pass; all pre-existing loop tests still
green via the global-seam fallback; one commit.
