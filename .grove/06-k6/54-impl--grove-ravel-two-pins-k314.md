# grove-ravel-two-pins-k314

## Goal

Replace Grove's local Ravel 2.0 record with the published 2.0.0 archives.
Then pass Grove's admission and installed checks against them and the
`ravel@2` prefix.

## Context

- `docs/ravel-target-inputs.md`, section "Rollout handoff: replace the local
  2.0 record". Its eleven steps are this leaf's procedure.
- `docs/grove-dual-ravel-stage-record.md`, section "Publication order", step 3.
- The publication receipts of ravel-two-publication-k313 in
  `docs/coordinated-release-evidence.md`.

## Done when

- The `"2.0"` lane of `scripts/ravel_target_inputs.py` names stem `2.0.0`,
  the three measured digests, the release download directory, the peeled
  commit of `v2.0.0` and the published build identity. `unrecorded` is
  empty. Each measured digest equals the digest that GitHub reports.
- The two tables of `docs/ravel-target-inputs.md` hold the published values.
  The 1.1.0 pins stay.
- These pass against the published archives and the `ravel@2` prefix:
  `task ravel:targets:controls`, `task ravel:targets:smoke`,
  `task release:smoke`, `task release:formula:test`,
  `task release:installed` and `task ravel:directory`.
- `task seam:shared-directory` passes with the published Ravel archives. This
  is the repeat that shared-directory-seam-k294 requires.
- `bash scripts/check.sh` passes on the working result.
- The stage record and the acceptance record's "Known limits" no longer say
  that the Grove pins wait for publication.

## Notes

The published Linux archives must be built at glibc 2.17, as the recorded
build was. If they are not, stop and record the difference. Do not edit the
pins to pass.

A change under `plugins/grove` or to `plugins/packs.json` changes core's
source digest and needs a new candidate keg. This leaf needs neither.
