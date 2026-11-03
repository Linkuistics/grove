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

## Decisions (running log)

- **Subjects.** Grove `9b8b69244125`, with an empty working copy above it.
  The 2.0 build is a `task install` from Ravel's output commit
  `71f70fcd3f92` into a scratch prefix. Its `bin/ravel` SHA-256 is
  `26b487af…`, its identity `a4f55d04…` occurs twice, and the prefix has no
  `libexec`. The 1.1 keg's `bin/ravel` SHA-256 is `c52f990b…`.
- **The real release-notes run uses a scratch HOME.** The owner's
  `policy.ts` routes `release-notes` to Claude Code, and this leaf must not
  change it. The run uses Grove's sample `policy.ts`, the 1.1 keg on PATH and
  `CODEX_HOME` at the owner's Codex home, as the helper states. It runs after
  the gates, because it builds into the shared target directory and rewrites
  `CHANGELOG.md`. The reviewed Unreleased text is then restored.
- **The stop-attempt lines come from every end on macOS.** Ravel kills the
  child's group twice while the unreaped child holds the group ID
  (`supervise_reported` in `keyed-launch/src/run.rs`). macOS answers EPERM
  for a group of zombies. Ravel tolerates that after it finds the group gone.
  A probe with a fake `/bin/sh` harness printed two lines in each of nine
  runs. The harness exited at once without a result, exited after
  `end-session`, or waited for TERM after `end-session`. Each case ran on 1.1
  unconfined, 1.1 `--confine` and 2.0.
- **Both gates passed.** The recipe runs `mise exec bun@1.4.3 -- bash scripts/check.sh`
  in a scratch HOME. It passed 19 of 19 with the 1.1 keg, from
  2026-11-03T15:12Z to 15:21Z. It passed 19 of 19 with the 2.0 build and the
  `host-ravel-reached` guard, from 15:21Z to 15:30Z. Each run passed the same
  1,379 test names, and the comparison failed on a copy with one name
  removed. The digest of every tracked file outside `.grove/` and of both
  `bin/ravel` files was the same before and after each run. Each gate printed
  the two stop-attempt lines once, in the real `grove run` case of release
  preparation.
- **The real release-notes run completed.** `task release:notes` ran from
  2026-11-03T15:32Z to 15:33Z with the 1.1 keg and the sample `policy.ts`.
  The transcript shows `sandbox: workspace-write [workdir, /tmp, $TMPDIR]`
  and the work directory beneath the system temporary directory.
  `ravel end-session grove-continue` succeeded from that directory, and
  Grove reported `completed`. The run printed stop attempts 3 and 4 only.
  The owner's `auth.json` did not change. `GROVE_RELEASE_RUNTIME_READ` was
  set and had no effect. The writer rewrote reviewed entries, so this leaf
  restored the reviewed `CHANGELOG.md`. The generated text stays in the
  scratch evidence only.
- **The Linux release smoke passed.** `task release:smoke
  TARGETS=aarch64-unknown-linux-gnu` ran from 2026-11-03T15:34Z to 15:35Z.
  The floor container and QEMU each passed both lanes and the directory case.
  Admission accepts only the pinned 2.0 archive `331008d4…` of build
  `23e50ce90037`, so the run used it. That build predates Ravel's removal,
  and Grove sends it no `--confine`. The k145 build has no admitted Linux
  archive until rollout replaces the pins. The run printed no stop-attempt
  line.
- **The record is a section of the withdrawal evidence.** The handoff's
  checks are in `docs/confinement-withdrawal-evidence.md`, so their results
  go in a new section "Removal checks" there.
