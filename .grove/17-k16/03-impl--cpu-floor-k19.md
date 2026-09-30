# cpu-floor-k19

## Goal

Prove the CPU floor for both Linux targets by running the installed smoke under
user-mode emulation with the Nehalem x64 and Cortex-A53 arm64 CPU models. Wire
every delivery check into the release pipeline, and state the documented floors.

## Context

Bun's own baseline verification emulates `-cpu Nehalem` and `-cpu cortex-a53`,
as `runtime-evidence.md` records. User-mode emulation runs on the host kernel,
so the kernel stays a documented claim.

## Done when

- The smoke task's Linux cases also run under user-mode emulation with the
  respective CPU model, inside the glibc-2.17 userland.
- Positive control: a probe using an instruction beyond each model is seen to
  fail under that model. Use AVX2 for x64, and an Armv8.1-or-later instruction
  such as the LSE atomics for arm64. A CPU model that silently accepts newer
  instructions would otherwise pass everything.
- `task release` runs the archive-content assertions and the per-target
  installed smoke before anything is published. Failure stops the release with
  a recovery note in `docs/RELEASING.md`.
- The usage documentation and release notes state the kernel floor as Bun
  1.4.2's documented range, labelled documented rather than executed: 5.1 in
  its README, 3.10 on its installation page. They state the macOS 13.0 minimum
  too.
- The runtime evidence records the emulated runs and both controls.
- As this node's last act: check the node brief's `Done when`. If the compiled
  worker cannot meet a floor, stop and escalate under the worker ADR's reopen
  condition instead of retiring.
