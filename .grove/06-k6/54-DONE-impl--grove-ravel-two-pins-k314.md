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

## Decisions (running log)

- The published archives were measured on 2026-11-12 from a new empty
  directory, `target/ravel-archives-2.0.0-download`. Each `shasum -a 256`
  value equals the GitHub `digest` and the k313 receipt.
- Both Linux executables need at most `GLIBC_2.17` (ELF version records), so
  the floor holds and no stop applies.
- The build identity is
  `7f2ff6b3f5222c13b69b78b8cd2c33d1c7edaa6a8599cfb27ebfb59d209d8ec5`. A
  re-implementation of `crates/ravel/build.rs` over the git objects of
  `ed6cedbf9a14` produced it. All three executables and the installed
  `ravel@2` keg hold it. None holds the old local identity.
- The `release is None` refusal stays in the script for a future local
  record. Its unit test moves to a synthetic local lane, because the real
  2.0 lane now downloads.
- The 2.0.0 template has `@CLASS@` and `@KEG_ONLY@`, which the seam did
  not render. The seam now renders them as Ravel's unversioned channel
  (`Ravel`, no `keg_only` line), because its cases model unversioned
  `ravel`. The seam's input is Ravel's template at `ed6cedbf9a14`, saved
  outside the working copy.
