# grove-confinement-floor-removal-k152

## Goal

Remove the bubblewrap and confinement floor route. Keep the glibc and CPU floor
proof, and keep the directory case on the general route.

## Context

- The section "Linux floor evidence" of `docs/confinement-withdrawal-evidence.md`,
  with its list of consumers.
- `scripts/release-smoke*.sh`, `scripts/grove-floor-directory-case.sh`,
  `scripts/grove-installed-smoke.sh`, the floor scripts and `Taskfile.yml`.

## Done when

- The scripts, tests, Taskfile tasks and documents that the section lists are
  removed or changed.
- The directory case runs on the 2.0 lane of the general route without its
  confinement probes, grants and unconfined control.
- `scripts/check.sh`, `docs/RELEASING.md`, `docs/ravel-target-inputs.md`,
  `docs/linux-floor-qemu.md` and `scripts/floor-seccomp/README.md` agree with
  the removal.
- `task release:floor:seccomp:controls`, `task release:test` and the link test
  `every_repository_markdown_reference_resolves` pass.
- The change is a described commit.

## Notes

The dated floor records stay as historical records. The one
`task release:smoke` run is in grove-confinement-removal-checks-k154.
