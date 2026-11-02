# grove-dual-target-inputs-k101

## Goal

Add an explicit 2.0 lane to Grove's pinned independent Ravel inputs, target
smoke and Linux floor checks. Keep the 1.1.0 lane and legacy-driver continuity.

## Context

- `scripts/ravel_target_inputs.py`, its tests, `scripts/ravel-target-smoke.sh`,
  `scripts/grove-floor-native-cases.sh` and the floor tasks.
- `scripts/legacy_driver_rehearsal.py` and `docs/legacy-driver-transition.md`.
- Ravel's `task release:archives` builds exact unreleased target archives with
  no version cut.

## Done when

- The target inputs keep the 1.1.0 pins. A second lane accepts 2.0 archives
  identified by source commit and build identity. Before the cut, these come
  from `task release:archives` at the directory build.
- The 2.0 lane refuses an archive whose digest or identity differs from its
  record. It never tests whatever the PATH finds.
- The target smoke and the Linux floor cases run each lane with the configuration
  format of its runtime. Each target that does not run is named.
- `legacy:controls` passes. `legacy:rehearsal` passes with the 1.1.0 keg while
  a 2.0 prefix is also installed. The ending executable of the old driver stays
  usable after each package step.
- A written handoff states how rollout replaces the local 2.0 identities with
  the published archives' SHA-256 before Grove's release.

## Notes

Never change `/opt/homebrew/Cellar/ravel/1.1.0`. It is the ending executable
of the live driver. Build archives in scratch directories.

Controls: an archive with one changed byte must refuse. A 2.0 lane that writes
`policy.ts` must fail its smoke. Watch each fail once.

From grove-directory-confinement-k99: the floor cases now hold an
unreadable-native-guidance case for 1.1.0. Run it on the 2.0 lane with
`config.rhai`. No floor measures the confined directory case on Linux until
that lane exists. Port the macOS case in `crates/grove/tests/directory_route.rs`
to the 2.0 lane, or name it as unmeasured.

On 2026-11-03,
`release:floor:native:run` refused a freshly prepared aarch64 bubblewrap,
SHA-256 `d35694e8915a70cf6ef403639c4d6c78c82c5cf0d1cc32ea6f1c6e93ca026cca`. The
recorded value in `docs/linux-floor-backend.md` is `21ad63fa…`. The QEMU floor
passed with its own prepared backends.
