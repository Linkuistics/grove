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

## Decisions (running log)

- The directory case runs from `release-smoke-target.sh`, after the installed
  cases, on a Linux target with the 2.0 lane. `smoke_linux` stages it as
  `directory-case.sh` only for Grove's cases, so the Ravel consumers that
  source `release-smoke.sh` stage nothing new.
- `floor_seccomp.py` keeps one profile, the QEMU profile, and loses `--qemu`.
  The native profile had no consumer after `release-floor-native.sh` went.
- The QEMU route drops `systempaths=unconfined`. A probe in the helper image
  with the pinned profile and Docker's masked paths mounted binfmt_misc and
  bound `/proc` and `/dev` in a private user and mount namespace. Only a new
  procfs mount was refused, and only bubblewrap made one.
- One `task release:smoke TARGETS=aarch64-unknown-linux-gnu` run on
  2026-11-04, after the script edits, passed without `systempaths=unconfined`.
  The floor container and QEMU each ran both lanes and the directory case.
  This run verifies the scripts. The recorded run stays in
  `grove-confinement-removal-checks-k154`.
- `docs/grove-delivery-contraction.md` said the floor environment supplies
  real confinement. Its link pointed at the retitled QEMU document, so this
  leaf changed that one sentence. `k153` keeps the rest of that document.
- `bash scripts/check.sh` with the 1.1 keg passed 18 of 19 checks. "Ravel
  caller assets and provenance" refused Bun 1.4.4 on PATH, because the
  typecheck pins 1.4.3. That check then passed with mise's Bun 1.4.3 first on
  PATH. The run with the 2.0 build stays in `k154`.
