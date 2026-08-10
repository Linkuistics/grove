# legacy-launch-cleanup-k83

**Kind:** impl

## Goal

Contract dead legacy-launch modules, tests, fixtures, and release metadata after
their public and routing behavior has been removed.

## Context

- Depends on `routing-policy-removal-k82`.
- Delete only artifacts proven unreachable by the two earlier increments.
  Methodology files `content/prompts/start.md` and `retire.md` remain owned by
  `lifecycle-methodology-k79`.
- Do not remove composition relationships or review evidence;
  `legacy-review-removal-k47` owns that contraction.

## Done when

- Dead launch modules, exports, dependencies, fixtures, and release metadata
  are removed without weakening provisioning or the bare driver.
- Removed-surface checks enumerate and classify candidates, with positive and
  cross-tree controls rather than a hand-picked pattern list.
- `cargo fmt --check` and `cargo test --locked` pass from the contracted tree.

## Notes

This cleanup is independently verifiable because no live behavior depends on
the deleted surfaces after `routing-policy-removal-k82`.
