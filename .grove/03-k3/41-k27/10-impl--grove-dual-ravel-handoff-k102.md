# grove-dual-ravel-handoff-k102

## Goal

Close grove-dual-ravel-k27 with full checks against each explicit build. Hand
the stage record to common-loading-acceptance-k29 and discovery-rollout-k6.

## Context

- The Done when of the node brief and of `common-skill-loading-k3`.
- The running logs of the leaves before this one.
- Tap states A to D and the publication order of
  `docs/specs/coordinated-release.md`.

## Done when

- `bash scripts/check.sh` passes with the 1.1 build. It also passes with the
  2.0 build and its runtime selected explicitly. Each run uses its own HOME.
- `ravel:resolution`, `ravel:commands`, `ravel:policy`, `ravel:activation`,
  `ravel:fixtures` and core conformance pass for each build. The record names
  each run's explicit build.
- A stage record in Grove's evidence names the Grove commits of this stage. It
  also names each build commit, build identity and `bin/ravel` SHA-256.
- The record gives reproducible recipes for each build prefix, each HOME, each
  configuration and settings file and each local source.
- The record states the publication order for acceptance and rollout. k29
  measures the unpublished build. Rollout publishes `ravel@2` before compatible
  Grove. It repeats admission and installed checks against the published
  archives before Grove's release. Promotion comes last, in finish.
- Every Done when of the node brief is met by a recorded result, or it becomes
  precise repair work before this node closes.
- The briefs of k29 and k6 name the stage record.

## Notes

Review each earlier leaf's record before deciding which checks need another
run. A green total is not a per-item confirmation.
