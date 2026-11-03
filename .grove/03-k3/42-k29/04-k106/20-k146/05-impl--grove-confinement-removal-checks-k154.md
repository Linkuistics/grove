# grove-confinement-removal-checks-k154

## Goal

Run the checks of the design against the state after the removal, and record
each result.

## Context

- The "Checks" list of the handoff in `docs/confinement-withdrawal-evidence.md`.
- The "HOME of each run" recipe of `docs/grove-dual-ravel-stage-record.md`.
- The Ravel build of ravel-confinement-removal-k145, from
  `scripts/harness-trials/builds.json`.

## Done when

- `bash scripts/check.sh` passes with the 1.1 keg and with the 2.0 build.
- One real `grove run release-notes` records that Codex runs under
  `workspace-write` and that the ending command reaches Ravel.
- One `task release:smoke` run with `TARGETS=aarch64-unknown-linux-gnu`
  passes the directory case. The record names the 2.0 archive.
- A record section names each subject by commit and build identity, and each
  result.

## Notes

Do not publish a release or change the owner's settings. Keep the 1.1.0 keg
and its ending command unchanged.

In grove-confinement-caller-removal-k150, each unconfined run of
`scripts/release-prepare.test.sh` printed two Ravel lines such as
`ravel: stop attempt 1 target -2447 signal 9: Operation not permitted`, on
1.1 and on 2.0. The run still completed. Record whether a confined run
printed them too, and whether a harness that ends normally causes them.
