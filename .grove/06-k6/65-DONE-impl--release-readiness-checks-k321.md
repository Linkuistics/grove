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

## Decisions (running log)

- The "published commit" of each affected repository is its remote `main`
  on 2026-11-13. k319 published documentation commits there. Each pack's
  source directory has the same tree ID at its pin and at that `main`. So
  the pack check at the pins and the repository checks at `main` read the
  same source bytes.
- The tap has no check suite of its own. This leaf runs `brew test` on each
  installed tap formula as the gate. It records `brew style`, `brew audit
  --strict` and `brew readall` as advisory, because no repository procedure
  gates on them and earlier records accepted the same offence classes.
- Grove's check runs once per Ravel lane, by the recipe of
  `docs/grove-dual-ravel-stage-record.md`: the 1.1.0 keg, then the published
  `ravel@2` keg with the guard `ravel` first on PATH.
- The 2.0 lane at `fe475b42725a` failed "release preparation" after its
  assertions passed. The published `ravel@2` keg names the shared default
  directory, so a HOME without settings caches the four installed skill
  kegs as read-only snapshots. The test's plain `rm -rf` trap then failed
  and gave its status to the script. The fix uses the repository's
  existing `chmod -R u+w` cleanup. It is in this leaf, because it is a
  one-line test fix with a seen-failing control. Both lanes run again on
  the fix commit.
- A source comparison with `v22.8.0` shows the launch-directory and
  admission contract. The finish needs no helper preservation.
