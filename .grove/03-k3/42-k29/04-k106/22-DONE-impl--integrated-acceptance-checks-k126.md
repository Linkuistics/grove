# integrated-acceptance-checks-k126

## Goal

Run the applicable Ravel checks, Grove core conformance and Grove's full
`bash scripts/check.sh` against the integrated subjects.

## Context

- The check decisions of `stage-evidence-reconciliation-k125` in the record.
- `docs/RELEASING.md`, the Ravel Taskfile and the Grove Taskfile.

## Done when

- Each check passes, or its failure becomes repair work.
- The record names each subject by commit and build identity, and the command
  and the result of each check.

## Notes

Do not publish a release, install `ravel@2` or change the owner's settings.

The "Check decisions" table of the record names each check, its subjects and
the reason to run it. Its "An observation for k126" section describes a
timeout test that failed on the 1.1 keg under load in k125.

## Decisions (running log)

- At entry on 2026-11-04, `jj git fetch` changed nothing. Ravel `main` is
  `24a545236185` and the bookmark is `71f70fcd3f92`. Grove `main` is
  `238c3a839ab4`. So Ravel's own stage checks do not run again, by the rule
  of the "Check decisions" table.
- The Grove subject is commit `40934ef71940`, the commit of k125, with an
  empty working copy above it.
- `task install` from the bookmark gave `bin/ravel` SHA-256 `26b487af…`, with
  the identity `a4f55d04…` twice and no `libexec`. The scratch prefix is in
  this session's scratchpad.
- The 22.1.0 keg of the rehearsal is the published archive `b45a030c…`. Its
  four executables match `docs/legacy-driver-inventory.json`, as in k101.
- The runs follow the recipe of the dual-Ravel stage: a 1.1 lane, a 2.0 lane
  with the guard `ravel`, and an installed lane. A fourth lane runs Ravel's
  `smoke:pair` and `release:smoke` in the Ravel workspace. The lanes run one
  after the other, because the machine load caused the timeout failure of
  k125. Each lane removes the `GROVE_*`, `RAVEL_*`, `HARNESS_*`, `CLAUDE*`
  and `CODEX*` names of this session first.
- The first 2.0 gate failed "Ravel caller assets and provenance". `check.sh`
  called `task ravel:review` with `RAVEL` only. The guards of `ravel:policy`
  read `RAVEL_RUNTIME`, so the 2.0 front ran the 1.1 typecheck and found no
  `libexec/ravel/sdk`. A dry run showed that an exported `RAVEL_RUNTIME=2.0`
  skips the typecheck, so k154's shell must have exported it. The recipe of
  the stage record sets only `GROVE_TEST_RAVEL_RUNTIME`, so it fails as
  written. This leaf repairs it: `check.sh` forwards
  `RAVEL_RUNTIME=${GROVE_TEST_RAVEL_RUNTIME:-}`. It is a one-line repair with
  the gate as its executable seam, so no separate repair leaf is cut.
- The lanes stopped at that point and run again from the start on the
  repaired commit. The logs of the first attempt are the failing control.
- On Grove `a81b1acc2d1d`, all four lanes passed every check from 16:18Z to
  16:45Z. Every digest before and after each lane equalled the start digest.
  The two gates passed the same 1,348 names, and a copy with one name removed
  made the comparison fail. The guard `ravel` exited 86 when called directly,
  and no log reached it.
- The record's new "Integrated acceptance checks" section names each subject,
  command and result. The stage record's recipe and the k125 directory row now
  point at the repair and at the k126 results.
