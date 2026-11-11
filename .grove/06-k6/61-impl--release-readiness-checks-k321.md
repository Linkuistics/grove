# release-readiness-checks-k321

## Goal

Run the checks of every affected repository against the published Ravel
2.0.0. State each acceptance limit before the finish handoff.

## Context

- The parent brief, Done when, second and third items.
- The root brief, "Agreed test seams".
- `docs/common-loading-acceptance-record.md`, "Known limits".
- `docs/owner-skill-delivery-evidence.md`, the staging acceptance of
  owner-staging-acceptance-k252.

## Done when

- `task check:packs` passes for core and each of the five domain packs, alone
  and in the aggregate with core, at the accepted commits.
- Each affected repository's own checks pass at its published commit: Ravel
  `main`, the six pack repositories, TestAnyware and the tap.
- `bash scripts/check.sh` passes on Grove's working result against the
  published Ravel 2.0.0 archives and the retained 1.1.0 pins.
- A test or a source comparison shows that the compatible Grove keeps the
  22.8.0 launch-directory and admission contract. The installed
  `grove-llm record-teardown` must admit under the running 22.8.0 driver. A
  change to that contract instead carries the helper preservation of
  RELEASING.md's "Signal-contract cutover" as live work.
- The record summarises the real Claude Code and Codex discovery evidence of
  the staged installed split, with resource and delegate reads and absent
  native copies. It names the build each trial used and each limit.
- Ravel's live book keeps the packaging guidance of
  ravel-packaging-guidance-k301.
- A committed record holds each command, its subject commit and its result.
  A failure is carried as a live leaf, not a completion claim.

## Notes

The repeat of `bash scripts/check.sh` after final main integration belongs to
finish.
