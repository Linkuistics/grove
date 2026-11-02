# grove-linux-two-lane-k103

## Goal

Make Ravel's directory build link at the glibc 2.17 floor. Record its Linux
archives in the 2.0 lane, and run that lane on the Linux floors. Without this
repair, no Linux 2.0 archive exists, and Ravel 2.0.0 cannot be cut, because
Ravel's own release builds all three targets.

## Context

- grove-dual-target-inputs-k101 found the defect on 2026-11-03. `task
  release:archives` in `Ravel.skill-directory-delivery` at output commit
  `4f831feb4d0f` built macOS. Both Linux targets failed to link:
  `ld.lld: error: undefined symbol: renameat2`, referenced by
  `ravel::skills::dir::Dir::rename_noreplace`. glibc added `renameat2` in 2.28,
  and the release links Linux against glibc 2.17 through `cargo zigbuild`.
- The 2.0 lane record is in `scripts/ravel_target_inputs.py`. It records the
  macOS archive and states this reason for both Linux targets.
  `docs/ravel-target-inputs.md` holds the lane, its tables and the rollout
  handoff.
- k101 gave the 2.0 lane to the target smoke and to the macOS release smoke.
  The Linux routes still run the 1.1 lane only. These are the routes:
  `release-smoke.sh` (`stage_linux_inputs`, `smoke_linux`),
  `release-smoke-target.sh` (it reads `RAVEL_2_TARGET_ARCHIVE` only on macOS),
  `release-floor-native.sh`, `release-floor-qemu.sh`,
  `release-smoke-qemu.sh` (`assert_archive_matches` requires the 1.x worker
  `libexec/ravel/ravel-policy`, and a 2.0 prefix has none) and
  `grove-floor-native-cases.sh` (three arguments, `policy.ts` only).
- `docs/linux-floor-native.md`, section "Runtime lanes", names the floor cases
  that no run measures on the 2.0 lane.

## Done when

- Ravel's directory build links at glibc 2.17 on both Linux targets. It keeps
  the behaviour of `renameat2` with `RENAME_NOREPLACE`. Settle the behaviour on
  a kernel or file system without that flag against the Ravel contract.
  Ravel's applicable checks pass, including
  `task test:native-linux`. The Ravel design evidence records the new build
  commit, output commit and build identity.
- The 2.0 lane records all three archives of that build, with the new source
  commit and identity, and `unrecorded` is empty. The Ravel builds section of
  the node brief names the new build.
- The target smoke runs the 2.0 lane natively in the floor container and under
  QEMU for each Linux target. The QEMU registration check accepts a 2.0 prefix
  with no worker and still refuses an incomplete 1.1 pair.
- The native and QEMU floor cases run the 2.0 lane with `config.rhai`. The
  cases are the caller lifecycle, standalone and imports, the sandbox denial
  task and the unreadable native guidance case. Port the confined directory case of
  `crates/grove/tests/directory_route.rs` to the Linux floor, or name it as
  unmeasured with the reason.
- Controls, each seen to fail once: a 2.0 floor lane that writes `policy.ts`
  fails. A Linux 2.0 archive with one changed byte refuses.

## Notes

The node brief says "Do not move the bookmark here" for the k27 stages. This
leaf is the exception: the repair is a Ravel stage like
ravel-local-sources-k90. Add the repair as a new commit above `4f831feb` on the
local bookmark `skill-directory-delivery`, and do not push it. Rebase only if
Ravel `main` moved, and record the bases as k90 did.

The release smoke keeps 1.1 and 2.0 prefixes apart by directory. Before the
cut, the archives of both lanes have the name `ravel-v1.1.0-TARGET.tar.xz`.
Keep that separation in the Linux mounts.

From k101: `release:floor:native:run` refused a freshly prepared aarch64
bubblewrap on 2026-11-03. Its SHA-256 was `d35694e8…`, and
`docs/linux-floor-backend.md` records `21ad63fa…`. Settle that before the
native floor can run.
